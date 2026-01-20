/*2052526 信15 白俊豪*/
#include <stdio.h>
#include <windows.h> //取系统时间
int main()
{
    LARGE_INTEGER tick, begin, end;

    QueryPerformanceFrequency(&tick); //获得计数器频率
    QueryPerformanceCounter(&begin);  //获得初始硬件计数器计数

    int i, j, k;
    int i1, i2, i3, j1, j2, j3, k1, k2, k3;
    int count = 0;
    int sum;

    for (i = 100; i < 1000; i++)
    {
        for (j = 100; j < 1000; j++)
        {
            for (k = 100; k < 1000; k++)
            {
                i3 = i % 10;
                i2 = (i - i3) / 10 % 10;
                i1 = i / 100;
                j3 = j % 10;
                j2 = (j - j3) / 10 % 10;
                j1 = j / 100;
                k3 = k % 10;
                k2 = (k - k3) / 10 % 10;
                k1 = k / 100;
                if (i1 != 0 && i2 != 0 && i3 != 0 && j1 != 0 && j2 != 0 && j3 != 0 && k1 != 0 && k2 != 0 && k3 != 0)
                {
                    if (i1 != i2 && i1 != i3 && i2 != i3 && j1 != j2 && j1 != j3 && j2 != j3 && k1 != k2 && k1 != k3 && k2 != k3 && i1 != j1 && i1 != j2 && i1 != j3 && i1 != k1 && i1 != k2 && i1 != k3 && i2 != j1 && i2 != j2 && i2 != j3 && i2 != k1 && i2 != k2 && i2 != k3 && i3 != j1 && i3 != j2 && i3 != j3 && i3 != k1 && i3 != k2 && i3 != k3 && j1 != k1 && j2 != k1 && j3 != k1 && j2 != k1 && j2 != k2 && j2 != k3 && j3 != k1 && j3 != k2 && j3 != k3)
                    {
                        if (i < j && j < k)
                        {
                            sum = i + j + k;
                            if (sum == 1953)
                            {
                                {
                                    count++;
                                    printf("No.%-3d : %d+%d+%d=1953\n", count, i, j, k);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    printf("total =  %d\n", count);
    QueryPerformanceCounter(&end); //获得终止硬件计数器计数

    printf("计数器频率 : %lld Hz\n", tick.QuadPart);
    printf("计数器计数 : %lld\n", end.QuadPart - begin.QuadPart);
    printf("%6lf秒\n", (double)(end.QuadPart - begin.QuadPart) / tick.QuadPart);

    return 0;
}
//Done.