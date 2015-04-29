import java.util.HashMap;
import java.util.List;

public class MyHashMap<K, V> extends HashMap<K, V> {

	/**
	 * 
	 */
	private static final long serialVersionUID = 962145404455470638L;

	// What is the K right over there? I need to figure that out. And also write
	// this. Just do a reduce thing, it'll be nice.
	public boolean findAllInKeys(List<K> l) {
		boolean out = true;
		for (K i : l) {
			out = out && this.containsKey(i);
		}
		return out;
	}

}
