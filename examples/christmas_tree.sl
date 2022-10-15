3 "N_SPACES" const // number of spaces (rows of stars)

1 // number of stars
N_SPACES 

{
	pop

	{
		" " print pop
	}
	switch for
	
	switch

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
} 1 N_SPACES - for