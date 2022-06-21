// Prints fizz if number is devisable by 3 and buzz if number is devisable by 5.

1

{
	copy
	{"fizz" print pop}
	switch 3 switch %
	// if the remainder from dividing by 3 is 0
	! if

	copy
	{"buzz" print pop}

	// if the remainder from dividing by 5 is 0
	switch 5 switch %
	! if

	copy
	{print}
	// if the remainder is not 0 for both 3 and 5
	switch

	copy 5 switch % switch
	3 switch % 
	&& if

	"" println

	pop

	1 +
}

101 for