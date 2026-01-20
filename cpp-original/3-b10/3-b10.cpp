/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;
int main()
{
    int x;
    while (1)
    {
        cout << "请输入x的值(0-100)：";
        cin >> x; //读入x的方式必须是cin>>int型变量
        while (cin.fail())
        {
            cin.clear();
            cin.ignore(1024,'\n');
            cout << "请输入x的值(0-100)：";
            cin >> x;
        }
        if (x >= 0 && x <= 100)
            break;
    }
    cout << "x=" << x << endl;
    return 0;
}
