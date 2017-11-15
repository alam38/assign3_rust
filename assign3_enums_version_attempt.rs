//Assignment 3

//assert_eq!(4, add_two(2)); // expected result, func call

/*fn test_interp() { 
	
	//assert_eq!(4, 2+2);
	//assert_eq!(6, interp(ExprC::numC(4)) );
	//assert_eq!(ExprC::numC{val: 4}, ExprC::numC{val: 4});
}*/	

enum ExprC { 

	numC { 
		val : i64
	},	

	idC { 
		id: char
	},	

	trueC { 
		val: bool
	},	

	falseC { 
		val: bool
	},	

	lamC { 
		params: Vec<char>,
		body: Box<ExprC>
	},	

	ifC { 
		test: Box<ExprC>,
		then: Box<ExprC>,
		last: Box<ExprC>
	},	

	binopC { 
		
		op: char,
		left : Box<ExprC>,
		right : Box<ExprC>
	},	

	appC { 

		func : Box<ExprC>,
		args : Vec<Box<ExprC>>
	}

}	

struct Binding { 
	name: char,
	val: Value
}


struct Env { 
	bindings: Vec<Binding>
	
}	

//return values for interpreter
enum Value { 

	numV { 
		n: i64

	},

	boolV {
		b: bool
	},	

	closV { 

		args: Vec<char>,
		body: Box<ExprC>,
		env: Env
	}

}	

fn main() { 
	
	//test_interp();	

}	

fn interp(expr: ExprC, env: Env) -> Value {

	match expr {
		
		ExprC::numC{val: v} => Value::numV{n: v},
		ExprC::idC{id: c} => lookup(c, env),
		ExprC::trueC{val: v} => Value::boolV{b: v},
		ExprC::falseC{val: v} => Value::boolV{b: v},
		ExprC::ifC{test: t, then: h, last: l} => interp_if(unsafe{*Box::into_raw(t)}, unsafe{*Box::into_raw(h)}, unsafe{*Box::into_raw(l)}, env)
        }

	/*trueC {	
                val: bool
        },

	falseC {
                val: bool
        },

	lamC {
              	params: Vec<char>,
                body: Box<ExprC>
        },

	ifC {
             	test: Box<ExprC>,
                then: Box<ExprC>,
                last: Box<ExprC>
        },


	}*/

}

fn interp_if (test: ExprC, then: ExprC, last: ExprC, env: Env) -> Value {

	let x = interp(test, env);

	match x {

		Value::boolV{b: v} => if v == true {
					 interp(then, env)	
				      }
				      else {
					 interp(last, env)
				      },
		_ => panic!("UIRE: 'interp_if test case did not evaluate into a boolean")
	}
}

fn lookup (sym: char, env: Env) -> Value {

	let length = env.bindings.len();

	for x in 0..length {
		let binding = env.bindings[x];
		if sym == binding.name {
			binding.val;
		}
	}

	panic!("UIRE: 'lookup Symbol is not in the environment");
}

