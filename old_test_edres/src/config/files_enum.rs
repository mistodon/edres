# ! [cfg_attr (rustfmt , rustfmt_skip)] # ! [allow (dead_code)] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , serde :: Serialize , serde :: Deserialize)] pub enum FileKey { FileA , FileB , FileC , } impl FileKey { pub const ALL : & 'static [FileKey] = & [FileKey :: FileA , FileKey :: FileB , FileKey :: FileC ,] ; } impl Default for FileKey { fn default () -> Self { Self :: FileA } } impl std :: fmt :: Display for FileKey { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "{:?}" , self) } } impl std :: str :: FromStr for FileKey { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { const STRINGS : & 'static [& 'static str] = & ["FileA" , "FileB" , "FileC" ,] ; for (index , & key) in STRINGS . iter () . enumerate () { if key == s { return Ok (FileKey :: ALL [index]) ; } } Err (()) } }