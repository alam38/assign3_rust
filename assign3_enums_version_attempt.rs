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
		
		ExprC::numC{val: v} => Value::numV{n: v}

	/*ExprCidC::expr.char
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
        },*/


	}

}
