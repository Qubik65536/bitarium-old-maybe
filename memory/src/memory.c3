module memory;

fn int add(int a, int b) @export("add_numbers")
{
	return a + b;
}


fn int sub(int a, int b) @export("sub_numbers")
{
	return add(a, -b);
}
