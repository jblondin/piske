#[macro_export]
macro_rules! define_func {
    ($name:ident, $env_name:ident, [$($vname:ident: $vtype:ty),*], $body:block) => {
        #[allow(unused_variables)]
        fn $name($env_name: &mut Environment, args: Vec<Value>) -> Result<Value, String> {
            #[allow(unused_imports)]
            use sindra::value::Extract;

            #[allow(unused_mut)]
            let mut arg_count = 0;
            $(
                let $vname: $vtype = args[arg_count].extract()?;
                arg_count += 1;
            )*
            if args.len() != arg_count {
                return Err(format!("incorrect number of arguments: expected {}, found {}",
                    arg_count, args.len()));
            }
            $body
        }
    }
}

#[macro_export]
macro_rules! add_func {
    ($scope:expr, $table:expr, $name:expr, $key:expr, $fn:ident,
            [$(($pname:expr, $ptype:expr)),*], $ret_ty:expr) => {{
        #[allow(unused_imports)]
        use ast::Parameter;
        use sindra::Identifier;
        #[allow(unused_imports)]
        use sindra::node::Node;
        use PType;

        let ident = Identifier($name.to_string());
        #[allow(unused_mut)]
        let mut params = vec![];
        $(
            params.push(Node::new(Parameter {
                name: Node::new(Identifier($pname.to_string())),
                ty: Node::new(Identifier($ptype.to_string()))
            }));
        )*
        $scope.define(ident.clone(), Symbol::ext_function(
            ident.clone(), Some($ret_ty), $key, params));
        $table.insert($key, Box::new($fn));
    }}
}

