5 "N_SPACES" const // number of spaces (rows of stars)

1 // number of stars
N_SPACES 

{
	pop

	copy
	
	{
		" " print pop
	}
	switch for

	switch

	copy
	{
		"*" print pop
	}
	switch for
	
	switch
	
	"" println pop

	1 switch -
	switch

	2 switch +
	switch

	copy 0 <=
} true while

{
	{
		" " print pop
	} N_SPACES for

	"*" println pop
} 2 N_SPACES / for