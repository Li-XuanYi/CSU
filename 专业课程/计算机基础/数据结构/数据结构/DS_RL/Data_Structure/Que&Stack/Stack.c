#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

// typedef struct ListNode
//     {
//         int val;
//         struct ListNode* next;
//     }Listnode;

// typedef struct
//     {
//         int size;
//         Listnode* top;
//     }Stack;

// Stack *newStack() // 构造函数
//     {
//         Stack* p = (Stack*)malloc(sizeof(Stack));
//         p->size = 0;
//         p->top = NULL;
//     }

// Stack *delStack(Stack* s) //析构函数
//     {
//         while(s->top)
//         {
//             Stack* temp = s->top->next;
//             free(s->top);
//             s->top = temp;
//         }
//         free(s);
//     }

// int SizeofStack(Stack* s) //获取栈大小
//     {
//         return s->size;
//     }

// bool isEmpty(Stack *s) {
//     return s->size == 0;
// }

// void push(Stack* s, int num) // 入栈
//     {
//         Listnode* p = (Listnode*)malloc(sizeof(Listnode));
//         p->val = num;
//         p->next = s->top;
//         s->top = p; // 更新栈顶
//         s->size++;
//     }

// void out(Stack* s) // 出栈
//     {
//         Listnode* temp = s->top;
//         s->top = temp->next;
//         free(temp);
//         s->size--;
//     }






int main()
{
    return 0;
}

