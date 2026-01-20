/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;
void haoni(int n, char src, char tmp, char dst)
{
    void move(int, char, char);
    if (n == 1)
        move(n, src, dst);
    else
    {
        haoni(n - 1, src, dst, tmp);
        move(n, src, dst);
        haoni(n - 1, tmp, src, dst);
    }
}


void move(int n, char x, char y)
{
    cout << n << "# ";
    cout << x << "-->" << y << endl;
}

int main()
{
    int level;
    char start, mid, end;
    while (1)
    {
        cout << "请输入汉诺塔的层数(1-16)" << endl;
        cin >> level;
        while (cin.fail())
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "请输入汉诺塔的层数(1-16)" << endl;
            cin >> level;
        }
        if (level >= 1 && level <= 16)
            break;
    }
    cin.clear();
    cin.ignore(1024, '\n');
    while (1)
    {
        cout << "请输入起始柱(A-C)" << endl;
        cin >> start;
        while (cin.fail())
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "请输入起始柱(A-C)" << endl;
            cin >> start;
        }
        if (start == 'A' || start == 'B' || start == 'C')
            break;
        else if (start == 'a' || start == 'b' || start == 'c')
        {
            start -= 32;
            break;
        }
        cin.clear();
        cin.ignore(1024, '\n');
    }
    cin.clear();
    cin.ignore(1024, '\n');
    while (1)
    {
        cout << "请输入目标柱(A-C)" << endl;
        cin >> end;
        while (cin.fail())
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "请输入目标柱(A-C)" << endl;
            cin >> end;
        }
        if (end == start || end == start + 32 || end == start - 32)
        {
            cin.clear();
            cin.ignore();
            cout << "起始柱不能与目标柱相同" << endl;
            continue;
        }
        if (end == 'A' || end == 'B' || end == 'C')
            break;
        else if (end == 'a' || end == 'b' || end == 'c')
        {
            end -= 32;
            break;
        }
        cin.clear();
        cin.ignore(1024, '\n');
    }
    mid = 'A' + 'B' + 'C' - start - end;
    haoni(level, start, mid, end);
}