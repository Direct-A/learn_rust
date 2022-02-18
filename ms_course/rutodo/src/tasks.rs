use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

// 返回文件操作结果
fn add_task(file: PathBuf, task: Task) -> Result<()> {
    // 读取文件，创建文件对象
    let mut file_obj = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file)?;

    // 从文件对象读取，并反序列化至数组包裹的 Task 结构体
    let tasks: Vec<Task> = match serde_json::from_reader(&file_obj) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        // 为什么需要 `?`
        Err(e) => Err(e)?,
    };

    // 序列化 Task 结构体
    tasks.push(task);

    // 写入文件
    file_obj.seek(SeekFrom::start(0))?;
    serde_json::to_writer(file, &file_obj)?;

    Ok(())
}

// fn del_task(file: PathBuf, task_pos: i32) -> Result<()> {}

// fn show_task(file: PathBuf) -> Result<()> {}
