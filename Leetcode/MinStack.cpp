class MinStack {
public:
    stack< pair<int, int> > s; // min, value
    void push(int x) {
        if(s.empty()){
            s.push(pair<int, int>(x, x));
        }
        else{
            s.push(pair<int, int>(min(s.top().first, x), x));
        }
    }

    void pop() {
        s.pop();
    }

    int top() {
        return s.top().second;
    }

    int getMin() {
        return s.top().first;
    }
};
