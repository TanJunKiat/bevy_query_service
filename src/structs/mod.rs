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

#[derive(Event, Debug, Clone)]
pub struct QueryEvent<T> {
    pub uuid: uuid::Uuid,
    pub request: T,
}

#[derive(Component, Debug, Clone)]
pub struct QueryRequest<T> {
    pub request: T,
}

#[derive(Component, Debug, Clone, Default)]
pub struct QueryReply<T> {
    pub reply: T,
}

#[derive(Component, Debug, Clone, Default)]
pub struct QueryFeedback<T> {
    pub feedbacks: Vec<T>,
}

#[derive(Component, Debug, Clone, Default)]
pub struct GoalComponent {
    pub uuid: uuid::Uuid,
    pub is_completed: bool,
    pub to_delete: bool,
    pub timer: bevy_time::Stopwatch,
}
