/*2052526 信15 白俊豪*/
//易知这一类数不可能为奇数
#include <stdio.h>
int main()
{
    int num;
    int factor;
    int sum = 0;
    for (num = 2; num <= 1000; num++)
    {
        sum = 0;
        for (factor = 1; factor < num; factor++)
        {
            if (num % factor == 0)
            {
                sum += factor;
            }
        }
        if (sum == num)
        {
            printf("%d,its factors are ", sum);
            for (factor = 1; factor < num; factor++)
            {
                if (num % factor == 0)
                {
                    if (factor < num / 2) //最后一个因子一定是该数的一半
                    {
                        printf("%d,", factor);
                    }
                    else
                    {
                        printf("%d", factor);
                    }
                }
            }
            printf("\n");
        }
    }
    return 0;
}