#include <stdio.h>
#include "area.h"

int CalculationArea(AreaStruct area) {
    printf(" ====>>> Area is %d print in c lib . \n" , area.widht * area.high);
    return area.high * area.widht;
}
