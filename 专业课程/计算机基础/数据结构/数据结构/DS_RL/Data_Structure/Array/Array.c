#include <stdio.h>

// int remove(int* a,int numsize,int val)
// {
//     int slow = 0;
//     for(int fast = 0; fast<numsize; fast++)
//     {
//         if(a[fast] != val)
//         {
//             a[slow++] = a[fast];
//         }
//     }
//     return slow;
// }

// int del(int* a,int size)
// {
// int slow = 0;
//     for(int fast = 1;fast<size; fast++)
//     {
//         if(a[fast] != a[slow])
//         {
//             slow++;
//             a[slow] = a[fast];
//         }
//     }
//     return slow + 1;
// }

int main()
{

    int a[10] = {1,1,2,2,2,3,3,4,5,5};

    printf("%d",del(a,10));
}