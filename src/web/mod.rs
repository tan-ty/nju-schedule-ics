/* HTTP服务器 */
pub mod server;
/* 解析命令行参数 */
pub mod config;
/* 登陆相关API
 * 与nju中的login不同，这里是给客户端
 * 登陆到本服务器的。
 */
mod login;
/* 让handler中可以使用`?`来处理错误 */
mod error;
/* 生成日历订阅文件 */
mod subscription;
/* 与redis进行通信 */
mod db;
