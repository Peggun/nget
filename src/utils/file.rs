// This file is part of nget
//
// Copyright (C) 2024 Peggun
//
// nget is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// nget is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.


use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use std::path::Path;

use crate::error::NgetError;

pub async fn save_to_file(content: &[u8], file_path: &str) -> Result<(), NgetError> {

    // Ensure the directory exists, not the file path itself
    let dir_path = Path::new(file_path).parent().unwrap();
    tokio::fs::create_dir_all(&dir_path)
        .await
        .map_err(|e| NgetError::FileError(format!("Failed to create directory: {}", e)))?;

    // Attempt to create and write to the file
    let mut file = File::create(file_path)
        .await
        .map_err(|e| NgetError::FileError(format!("Failed to save file: {}", e)))?;

    file.write_all(content)
        .await
        .map_err(|e| NgetError::FileError(format!("Failed to write to file: {}", e)))?;

    Ok(())
}
