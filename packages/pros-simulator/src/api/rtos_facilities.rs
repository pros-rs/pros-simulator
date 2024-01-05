//! RTOS Facilities API
//!
//! ## Reference
//!
//! * `delay`
//! * `millis`
//! * `micros` (not implemented)
//! * `mutex_create`
//! * `mutex_delete`
//! * `mutex_give`
//! * `mutex_take`
//! * `task_create`
//! * `task_delay`
//! * `task_delay_until` (not implemented)
//! * `task_delete` (not implemented)
//! * `task_get_by_name` (not implemented)
//! * `task_get_count` (not implemented)
//! * `task_get_current` (not implemented)
//! * `task_get_name` (not implemented)
//! * `task_get_priority` (not implemented)
//! * `task_get_state` (not implemented)
//! * `task_notify` (not implemented)
//! * `task_notify_clear` (not implemented)
//! * `task_notify_ext` (not implemented)
//! * `task_notify_take` (not implemented)
//! * `task_join` (not implemented)
//! * `task_resume` (not implemented)
//! * `task_set_priority` (not implemented)
//! * `task_suspend` (not implemented)
//!
//! ### FreeRTOS
//!
//! * `rtos_suspend_all`
//! * `rtos_resume_all`
//! * `pvTaskGetThreadLocalStoragePointer`
//! * `vTaskSetThreadLocalStoragePointer`
//! * `xTaskAbortDelay` (not implemented)

use std::{
    mem::size_of,
    time::{Duration, Instant},
};

use futures_util::Future;
use pros_sys::TIMEOUT_MAX;
use tokio::time::sleep;
use wasmtime::{Caller, Linker, SharedMemory, Store};

use crate::{
    host::{
        memory::SharedMemoryExt,
        task::{TaskOptions, TaskPool},
        thread_local::GetTaskStorage,
        Host, HostCtx, ResultExt,
    },
    system::system_daemon::CompetitionPhaseExt,
};

pub fn configure_rtos_facilities_api(linker: &mut Linker<Host>) -> anyhow::Result<()> {
    linker.func_wrap0_async("env", "mutex_create", |caller: Caller<'_, Host>| {
        Box::new(async move {
            let mutex_id = caller.mutexes_lock().await.create_mutex();
            Ok(mutex_id as u32)
        })
    })?;

    linker.func_wrap1_async(
        "env",
        "mutex_delete",
        |caller: Caller<'_, Host>, mutex_id: u32| {
            Box::new(async move {
                caller.mutexes_lock().await.delete_mutex(mutex_id as usize);
                Ok(())
            })
        },
    )?;

    linker.func_wrap1_async(
        "env",
        "mutex_give",
        |caller: Caller<'_, Host>, mutex_id: u32| {
            Box::new(async move {
                caller.mutexes_lock().await.unlock(mutex_id as usize);

                Ok(u32::from(true))
            })
        },
    )?;

    linker.func_wrap2_async(
        "env",
        "mutex_take",
        |caller: Caller<'_, Host>, mutex_id: u32, timeout: u32| {
            Box::new(async move {
                let timeout = (timeout != TIMEOUT_MAX)
                    .then(|| Instant::now() + Duration::from_millis(timeout.into()));
                let success = caller
                    .mutexes_lock()
                    .await
                    .lock(mutex_id as usize, timeout)
                    .await;
                Ok(u32::from(success))
            })
        },
    )?;

    linker.func_wrap2_async(
        "env",
        "pvTaskGetThreadLocalStoragePointer",
        |mut caller: Caller<'_, Host>, task_handle: u32, storage_index: i32| {
            Box::new(async move {
                let storage = caller.task_storage(task_handle).await;
                Ok(storage.get(caller.memory(), storage_index))
            })
        },
    )?;

    linker.func_wrap3_async(
        "env",
        "vTaskSetThreadLocalStoragePointer",
        |mut caller: Caller<'_, Host>, task_handle: u32, storage_index: i32, value: u32| {
            Box::new(async move {
                let mut storage = caller.task_storage(task_handle).await;
                storage.set(caller.memory(), storage_index, value)
            })
        },
    )?;

    linker.func_wrap0_async("env", "task_get_current", |caller: Caller<'_, Host>| {
        #[allow(clippy::let_and_return)]
        Box::new(async move {
            let current = caller.current_task().await;

            let id = current.lock().await.id();
            // fixing warning causes compile error
            id
        })
    })?;

    fn task_delay(
        _caller: Caller<'_, Host>,
        millis: u32,
    ) -> Box<dyn Future<Output = anyhow::Result<()>> + Send + '_> {
        Box::new(async move {
            if millis > 0 {
                let end = Instant::now() + Duration::from_millis(millis.into());
                while Instant::now() < end {
                    TaskPool::yield_now().await;
                }
            } else {
                TaskPool::yield_now().await;
            }

            Ok(())
        })
    }

    linker.func_wrap1_async("env", "delay", task_delay)?;
    linker.func_wrap1_async("env", "task_delay", task_delay)?;

    linker.func_wrap2_async(
        "env",
        "task_delay_until",
        |caller: Caller<'_, Host>, prev_time_ptr: u32, delta_ms: u32| {
            Box::new(async move {
                assert_ne!(prev_time_ptr, 0);
                assert!(delta_ms > 0);

                let epoch = caller.start_time();

                let memory = caller.memory();
                let u32_bits = memory.read_relaxed(prev_time_ptr as usize, size_of::<u32>())?;
                let prev_time = u32::from_le_bytes(u32_bits.try_into().unwrap());

                let end = epoch
                    + Duration::from_millis(prev_time.into())
                    + Duration::from_millis(delta_ms.into());

                TaskPool::yield_now().await;
                while Instant::now() < end {
                    TaskPool::yield_now().await;
                }

                Ok(())
            })
        },
    )?;

    linker.func_wrap0_async("env", "rtos_suspend_all", |caller: Caller<'_, Host>| {
        Box::new(async move {
            let mut tasks = caller.tasks_lock().await;
            tasks.suspend_all();
            Ok(())
        })
    })?;

    linker.func_wrap0_async("env", "rtos_resume_all", |caller: Caller<'_, Host>| {
        Box::new(async move {
            let mut tasks = caller.tasks_lock().await;
            let res = tasks.resume_all().await?;
            Ok(i32::from(res))
        })
    })?;

    linker.func_wrap0_async("env", "millis", |caller: Caller<'_, Host>| {
        Box::new(async move { Ok(caller.start_time().elapsed().as_millis() as u32) })
    })?;

    // task_t task_create ( task_fn_t function,
    //     void* parameters,
    //      uint8_t prio,
    //      uint16_t stack_depth,
    //      const char* name )
    linker.func_wrap5_async(
        "env",
        "task_create",
        |caller: Caller<'_, Host>,
         function: u32,
         parameters: u32,
         priority: u32,
         _stack_depth: u32,
         _name: u32| {
            Box::new(async move {
                let mut tasks = caller.tasks_lock().await;
                let opts =
                    TaskOptions::new_extern(&mut tasks, caller.data(), function, parameters)?
                        .priority(priority - 1);
                let task = tasks
                    .spawn(opts, &caller.module(), &caller.interface())
                    .await?;

                let task = task.lock().await;
                Ok(task.id())
            })
        },
    )?;

    Ok(())
}
