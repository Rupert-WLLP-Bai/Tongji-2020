/*2052526 信15 白俊豪*/
#include <iostream>
#include <cstring>
#include <cmath>
#define SIZE 33
using namespace std;

unsigned int f(char* a)
{
    char * p = a;
    char* start = a;
    unsigned int sum = 0;
    cout << "请输入一个0/1组成的字符串，长度不超过32" << endl;
    cin.getline(a, SIZE);

    while(*p!='\0')
    {
        unsigned int add = (unsigned int) pow(2, strlen(a) - 1 - (p-start)) * (unsigned int)(*p++ - '0');
        sum += add;
    }
    return sum;
}

int main()
{
    char a[SIZE];
    cout << f(a) << endl;
    return 0;
}