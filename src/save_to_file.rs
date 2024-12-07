use crate::*;


pub trait SaveToFile
{
    fn save_to_file_ron(&self, filename: &str) -> Result<(), &'static str> 
        where Self : Serialize
    {
        let pretty_config = PrettyConfig::default();
        let ron_string = to_string_pretty(self, pretty_config).map_err(|_| "Ron : Serialization failed")?;

        let mut file = File::create(&(filename.to_owned() + ".ron")).map_err(|_| "Ron : Can't create file")?;
        file.write_all(ron_string.as_bytes()).map_err(|_| "Ron : Can't write to file")?;
        Ok(())
    }
    
    fn load_from_file_ron(filename: &str) -> Result<Self, &'static str> where Self : for<'de> Deserialize<'de> {
        let file = File::open(&(filename.to_owned() + ".ron")).map_err(|_| "Ron : Can't open file")?;
        let data: Self = ron::de::from_reader(file).map_err(|_| "Ron : Can't read file")?;
        Ok(data)
    }

    fn save_to_file_bin(&self, filename: &str) -> Result<(), &'static str> where Self : Serialize
    {
        let serialized_data: Vec<u8> = serde_binary::to_vec(&self, Endian::Little).map_err(|_| "Bin : Serialization failed")?;
        std::fs::write(&(filename.to_owned() + ".bin"), serialized_data).map_err(|_| "Bin : Can't create/write to file")?;
        Ok(())
    }
    
    fn load_from_file_bin(filename: &str) -> Result<Self, &'static str> where Self : for<'de> Deserialize<'de> {
        let serialized_data = std::fs::read(&(filename.to_owned() + ".bin")).map_err(|_| "Bin : Can't open file")?;
        let data: Self = serde_binary::from_slice(&serialized_data, Endian::Little).map_err(|_| "Bin : Deserialization failed")?;
        Ok(data)
    }
}
// implémente le trait `SaveToFile` pour tout les types génériques T qui implémentent les traits suivants :
impl<T> SaveToFile for T where T : Serialize + for<'de> Deserialize<'de> {}
