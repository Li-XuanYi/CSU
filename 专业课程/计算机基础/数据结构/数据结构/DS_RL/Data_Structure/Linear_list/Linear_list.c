// 习题2.2.3 线性表

// T1
// #define size 5
// #include <stdio.h>
// #include <stdlib.h>
// #include <time.h>
// int main()
// {
//     int a[size];
//     srand((unsigned int)(time(NULL)));
//     for(int i=0;i<size;i++)
//     {
//         a[i] = rand(); //设置随机数
//     }
//     int j = 0;
//     int value = a[j];
//     for(j=1; j<size; j++)
//     {
//         if(value > a[j])        
//         {
//             value = a[j]; // 查找最小数
//         }
//     }
//     a[j] = a[size];
//     printf("%d",value);
//     for(int i =0;i<5;i++){
//     printf("NN%dNN",a[i]); // 打印结果
//     }
//     return 0;
// }

// T2 假设size=6
#include <stdio.h>

int main()
{
    int temp = 0;
    int L[6] = {11,23,13,64,54,69};
    for(int i=0,j=5; i<=j; i++,j--)
    {
        temp = L[i];
        L[i] = L[j];
        L[j] = temp;
    }
    for(int k=0;k<6;k++)
    {
        printf("%d\n",L[k]);
    }
    
    return 0;
}



//T5
// #include <stdio.h>
// #define size 13
// int main()
// {
//     int a[size] = {1,3,5,8,11,11,11,13,13,16,17,18,18};
//     for(int i=0,j=1;i<size;i++)
//     {
//         if(a[i] == a[j])
//         {
//             j++;
//             continue;
//         }
//         else if (a[i] != a[j])
//         {
           
//         }
//     }
// }



// T6
// #include <stdio.h>
// int* Add(int* a,int* b)
// {
//     int size_c = sizeof(a)+sizeof(b);
// }
// int main()
// {
//     return 0;
// }



