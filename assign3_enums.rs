//Assignment 3

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

	ifC { 
		test: <Box<ExprC>>,
		then: <Box<ExprC>>,
		last: <Box<ExprC>>
	},	

	lamC { 
		params: Vec<char>,
		body: <Box<ExprC>>

	},	

	binopC { 
		
		op: char,
		left : <Box<ExprC>>,
		right : <Box<ExprC>>
	},	

	appC { 

		func : <Box<ExprC>>,
		args : Vec< <Box<ExprC>> >
	}

}	

fn main() { 
	


}	
