/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;
int is_power(int num, int base)
{
    if (num == 1)
        return 1;
    if (num % base == 0)
        return is_power(num / base, base);
    else
        return 0;
}

int main()
{ 
    int num, base;
    cout << "请输入整数num及基数base(2以上的正整数)" << endl;
    cin >> num >> base;
    if (is_power(num, base))
        cout << num << "是" << base << "的幂" << endl;
    else
        cout << num << "不是" << base << "的幂" << endl;
    return 0;
}