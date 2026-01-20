/*2052526 ÐÅ15 °×¿¡ºÀ*/
#include <stdio.h>
int main()
{
    int i, j, result;
    int count = 10;
    for (i = 1; i < count; i++)
    {
        for (j = 1; j <= i; j++)
        {
            result = i * j;
            printf("%d¡Á%d=%-4d", j, i, result);
            //cout << j << "¡Á" << i << "=" << setiosflags(ios::left) << setw(4) << result;
        }
        printf("\n");
    }
    return 0;
}

//Done.