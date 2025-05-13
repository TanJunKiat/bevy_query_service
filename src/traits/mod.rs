// =========================================================================
/*
 * Copyright (C) 2019 Tan Jun Kiat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// =========================================================================
use super::*;

pub trait QueryServerOps<T> {
    fn get_reply(world: &mut World, request: &QueryRequest<T>) -> Result<Self>
    where
        Self: Sized;
}

pub trait QueryClientOps<T> {
    fn send_request(ctx: &mut bevy_tokio_tasks::TaskContext, request: &QueryRequest<T>) -> impl std::future::Future<Output = Result<Self>> + Send
    where
        Self: Sized;
}
