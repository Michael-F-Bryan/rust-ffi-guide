struct MyVec;
typedef struct MyVec MyVec;

MyVec* my_vec_new();
int my_vec_len(const MyVec*);
void my_vec_push(MyVec*, int);
int* my_vec_contents(MyVec*);
void my_vec_destroy(MyVec*);

