macro_rules! count_args {
    () => { 0 };
    ($_e:ty $(, $rest:ty)*) => { 1 + count_args!($($rest),*) }
}

#[macro_export]
macro_rules! add_interpreter_func {
    ($intrp_fn_name:ident, $fn_name:ident, [$($vtype:ty),*], $result_map:expr) => {
        #[allow(unused_variables)]
        fn $intrp_fn_name(env: &mut Environment, args: Vec<Value>) -> Result<Value, String> {
            #[allow(unused_imports)]
            use sindra::value::Extract;
            let arg_count = count_args!($($vtype),*);
            if args.len() != arg_count {
                return Err(format!("incorrect number of arguments: expected {}, found {}",
                    arg_count, args.len()));
            }

            #[allow(unused_mut)]
            let mut arg_num = 0;
            $fn_name(env, $({
                #![allow(unused_assignments)]
                let result: $vtype = args[arg_num].extract()?;
                arg_num += 1;
                result
            }),*).map($result_map)
        }
    }
}

#[macro_export]
macro_rules! add_func {
    ($scope:expr, $table:expr, $name:expr, $key:expr, $fn:ident,
            [$(($pname:expr, $ptype:expr)),*], $ret_ty:expr) => {{
        #[allow(unused_imports)]
        use ast::Parameter;
        #[allow(unused_imports)]
        use sindra::{Typed, Identifier};
        #[allow(unused_imports)]
        use sindra::node::Node;
        use PType;

        let ident = Identifier($name.to_string());
        #[allow(unused_mut)]
        let mut params = vec![];
        $(
            let node = Node::new(Parameter {
                name: Node::new(Identifier($pname.to_string())),
                ty: Node::new(Identifier($ptype.to_string()))
            });
            node.annotation.borrow_mut().set_type(Some(PType::from($ptype)));
            params.push(node);
        )*
        $scope.define(ident.clone(), Symbol::ext_function(
            ident.clone(), Some($ret_ty), $key, params));
        $table.insert($key, Box::new($fn));
    }}
}

