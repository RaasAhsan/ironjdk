public class Counter {
    int key;
    Counter left, right;

    public Counter(int item) {
        this.key = item;
        this.left = null;
        this.right = null;
    }

    public static void main(String[] args) {
        int[] array = new int[] { 0, 1, 2, 3 };

        array[0] = 3;

        int a = array[0];
        int b = -700;
        int c = 5;
        int d = a - b * c;
    }
}
