/*2052526 ÐÅ15 °×¿¡ºÀ*/
#include <iostream>
#include <iomanip>
using namespace std;
int main()
{
    int i, j,result;
    int count = 10;
    for (i = 1; i < count; i++)
    {
        for (j = 1; j <= i; j++)
        {
            result = i * j;
            cout<< j << "¡Á" << i << "=" << setiosflags(ios::left)<<setw(4)<<result;
        }
        cout << endl;
    }
    return 0;
}

//Done.