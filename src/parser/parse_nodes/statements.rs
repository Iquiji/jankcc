/*
(6.8) statement:
    labeled-statement
    compound-statement
    expression-statement
    selection-statement
    iteration-statement
    jump-statement
*/

/*
(6.8.1) labeled-statement:
    identifier : statement
    case constant-expression : statement
    default : statement
*/

/* 
(6.8.2) compound-statement:
    { block-item-list opt }
(6.8.2) block-item-list:
    block-item
    block-item-list block-item
(6.8.2) block-item:
    declaration
    statemen
*/

/*
(6.8.3) expression-statement:
    expressionopt ;
*/

/*
(6.8.4) selection-statement:
    if ( expression ) statement
    if ( expression ) statement else statement
    switch ( expression ) statement
*/

/*
(6.8.5) iteration-statement:
    while ( expression ) statement
    do statement while ( expression ) ;
    for ( expression opt ; expression opt ; expression opt ) statement
    for ( declaration expression opt ; expression opt ) statement
*/

/*
(6.8.6) jump-statement:
    goto identifier ;
    continue ;
    break ;
    return expression opt ;
*/