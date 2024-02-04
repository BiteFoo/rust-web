//保存在本地文件

use std::io::Error;
use std::path::{Path, PathBuf};
use tracing::debug;


pub struct LocalStorage {
    name: String,
}

//准备好一个文件路径，用于保存
async  fn make_sure_exits(dir: &str)->std::result::Result<String,Box<dyn std::error::Error>>{

    debug!("checking save dir {dir:<12}");
    let path = Path::new(dir);
    if ! path.is_dir(){
        std::fs::create_dir(path)?;
        debug!("create dir");
    }

    Ok(
        path.to_string_lossy().to_string()
    )
}

impl LocalStorage {
    pub fn new(save: &str)->Self{
        Self{
            name: save.to_string()
        }
    }

    async fn  get_save_root(&self)->Result<PathBuf,Error> {
        //在本地路径下创建
        let current_dir = std::env::current_dir()?;
        let current = current_dir.join("upload");
        make_sure_exits(current.to_str().unwrap()).await.unwrap();
        debug!("create cache root ok ");
        Ok(
            current
        )
    }
    pub  async fn create_save_file(&self)->std::result::Result<String ,Box<dyn std::error::Error>>{
        let save  = self.get_save_root().await?.join(self.name.clone());
        //获取创建的结果

        Ok(
            save.to_string_lossy().to_string()
        )
    }
}