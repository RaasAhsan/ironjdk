public class Counter {
    int key;
    Counter left, right;

    public Counter(int item) {
        this.key = item;
        this.left = null;
        this.right = null;
    }

    public static void main(String[] args) {
        int a = 0;
        int b = 4;
        int c = 5;
        int d = a - b * c;
    }
}
