/*
 * Copyright (C) 2019 Open Whisper Systems
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(non_snake_case)]

use serde_derive::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetricsConfig {
    pub reporters: Vec<MetricsReporterConfig>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, tag="type", rename_all="lowercase")]
pub enum MetricsReporterConfig {
    Json(JsonMetricsReporterConfig),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonMetricsReporterConfig {
    pub hostname: String,

    pub token: String,

    pub intervalSeconds: Option<u64>,
}