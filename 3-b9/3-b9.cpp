/*2052526 信15 白俊豪*/
//易知这一类数不可能为奇数
#include <iostream>
using namespace std;
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
            cout << sum << ",its factors are ";
            for (factor = 1; factor < num; factor++)
            {
                if (num % factor == 0)
                {
                    if (factor < num / 2) //最后一个因子一定是该数的一半
                    {
                        cout << factor << ",";
                    }
                    else
                    {
                        cout << factor;
                    }
                }
            }
            cout << endl;
        }
    }
    return 0;
}