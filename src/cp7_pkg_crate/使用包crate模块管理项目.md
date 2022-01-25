package：包含一个Cargo.toml，至少包含一个crate（最多一个library crate和任意个binary crate）
crate：一个二进制项或库
    src/main.rs是一个与其同名的library crate的crate root
module
path


src/main.rs -- 一个和package同名的binary crate，且这个是binary crate root
src/lib.rs -- 一个和package同名的library crate，且这个是library crate root
src/bin/test.rs -- 一个名为test的binary crate
src/bin/test1.rs -- 一个名为test1的binary crate


也可以把同一crate下的不同module分到不同文件中
以下示例为一个library crate：
${crate_name}/src/lib.rs -- 名为${crate_name}的library crate的root
${crate_name}/src/${module_name}.rs -- 名为module_name的一个module
${crate_name}/src/${module_name}/${module_name_2}.rs -- 名为module_name_2的module_name下的一个module
`mod module_name_2 {}`，直接写内部的函数结构体即可。