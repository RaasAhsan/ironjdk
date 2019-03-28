public class Counter {
    int key;
    Counter left, right;

    public Counter(int item) {
        this.key = item;
        this.left = null;
        this.right = null;
    }

    public static void main(String[] args) {
        Counter c = null;
        int a = 0;

        if (c == null) {
            a = 1;
        } else {
            a = 2;
        }
    }
}
