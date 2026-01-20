/*2052526 信15 白俊豪*/
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
            printf("%d×%d=%-4d", j, i, result);
            //cout << j << "×" << i << "=" << setiosflags(ios::left) << setw(4) << result;
        }
        printf("\n");
    }
    return 0;
}

//Done.