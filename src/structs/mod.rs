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
    uuid: uuid::Uuid,
    is_executing: bool,
    is_completed: bool,
    to_delete: bool,
    timer: bevy_time::Stopwatch,
}

impl GoalComponent {
    pub fn new(uuid: uuid::Uuid) -> Self {
        Self {
            uuid,
            is_executing: false,
            is_completed: false,
            to_delete: false,
            timer: bevy_time::Stopwatch::new(),
        }
    }

    pub fn get_uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    pub fn mark_executing(&mut self) {
        self.is_executing = true;
    }

    pub fn is_executing(&self) -> bool {
        self.is_executing
    }

    pub fn mark_completed(&mut self) {
        self.is_executing = false;
        self.is_completed = true;
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    pub fn mark_to_delete(&mut self) {
        self.is_executing = false;
        self.to_delete = true;
    }

    pub fn is_to_delete(&self) -> bool {
        self.to_delete
    }
}
