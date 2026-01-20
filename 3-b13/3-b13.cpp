/*2052526 信15 白俊豪*/
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <windows.h> //取系统时间
using namespace std;

int main()
{
    LARGE_INTEGER tick, begin, end;

    QueryPerformanceFrequency(&tick); //获得计数器频率
    QueryPerformanceCounter(&begin);  //获得初始硬件计数器计数

    int i, j, k;
    int i1, i2, i3, j1, j2, j3, k1, k2, k3;
    int count = 0;
    int sum;

    for (i = 123; i < 651; i++)
    {
        for (j = 123; j < 988; j++)
        {
            for (k = 123; k < 988; k++)
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
                                    cout << "NO." << setw(3) << count << " : " << i << "+" << j << "+" << k << "= 1953" << endl;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cout << "total = " << count << endl;

    QueryPerformanceCounter(&end); //获得终止硬件计数器计数

    cout << "计数器频率 : " << tick.QuadPart << "Hz" << endl;
    cout << "计数器计数 : " << end.QuadPart - begin.QuadPart << endl;
    cout << setiosflags(ios::fixed) << setprecision(6) << double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;

    return 0;
}
//Done.