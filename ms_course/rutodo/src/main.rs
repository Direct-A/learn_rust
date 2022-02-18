/*
A todo list project
For my first Rust Project

目标：
- [ ] 可添加新任务
- [ ] 可删除就任务
- [ ] 打印任务列表

*/


mod cli;
mod tasks;
use structopt::StructOpt;

fn main() {
    cli::CommandlineArgs::from_args();
}
