class Stack {
public:
    // Push element x onto stack.
    queue<int> q[2];
    int main;
    Stack() : main(0) {};
    void push(int x) {
        q[main].push(x);
    }

    // Removes the element on top of the stack.
    void pop() {
        while(!q[main].empty()){
                int x = q[main].front();
                q[main].pop();
                if(q[main].empty()) break;
                q[main^1].push(x);
        }
        main ^= 1;
    }

    // Get the top element.
    int top() {
        int res;
        while(!q[main].empty()){
                int x = q[main].front();
                q[main].pop();
                if(q[main].empty()){
                    res =  x;
                } 
                q[main^1].push(x);
        }
        main ^= 1;
        return res;
    }

    // Return whether the stack is empty.
    bool empty() {
        return q[main].empty();
    }
};
