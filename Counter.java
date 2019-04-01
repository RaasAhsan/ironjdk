public class Counter {
    int key;
    Counter left, right;

    public Counter(int item) {
        this.key = item;
        this.left = null;
        this.right = null;
    }

    public int set(int a, int b) {
        this.key = a + b;

        return this.key;
    }

    public static void main(String[] args) {
        Counter c = new Counter(3);
        c.key = 5;

        int a = c.key;

        c.set(4, 9);
    }
}
