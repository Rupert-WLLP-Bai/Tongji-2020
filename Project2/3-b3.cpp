/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;
int main()
{
    int i;
    int temp1, temp2, temp3, temp4;
    int i1, i2, i3, i4, i5;
    cout << "请输入一个[1..30000]间的整数：" << endl;
    cin >> i;
    i1 = i % 10;
    temp1 = i / 10;
    i2 = temp1 % 10;
    temp2 = temp1 / 10;
    i3 = temp2 % 10;
    temp3 = temp2 / 10;
    i4 = temp3 % 10;
    temp4 = temp3 / 10;
    i5 = temp4 % 10;

    cout << "万位 : " << i5 << endl;
    cout << "千位 : " << i4 << endl;
    cout << "百位 : " << i3 << endl;
    cout << "十位 : " << i2 << endl;
    cout << "个位 : " << i1 << endl;

    return 0;
}
//Done.
