package com.ddsnowboard.cartextresponder;

import android.app.NotificationManager;
import android.app.Service;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.os.IBinder;
import android.preference.EditTextPreference;
import android.support.v4.app.NotificationCompat;
import android.widget.Toast;
import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;

public class BluetoothService extends Service {
	 static EditTextPreference car;

	@Override
	public IBinder onBind(Intent arg0) {
		// TODO Auto-generated method stub
		return null;
	}	
	 public void onCreate() {
		 BluetoothAdapter adapter = BluetoothAdapter.getDefaultAdapter();
		 adapter.disable();
		 car = new EditTextPreference(this);
		 while(adapter.getState()!=10){
		 }
		 adapter.enable();
		 IntentFilter filter1 = new IntentFilter(BluetoothDevice.ACTION_ACL_CONNECTED);
		 this.registerReceiver(mReceiver, filter1);
//		 Set pairs = adapter.getBondedDevices();
//		 int size = pairs.size();
//		 for(Integer i=0; i<size;i++)
//		 {
//			 pairs.toArray().get(i).getName();
//		 }		 
//	       BluetoothAdapter adapter = BluetoothAdapter.getDefaultAdapter();
//	       if (adapter == null)
//	       {
//	    	   stopSelf();
//	       }
//	       else
//	       {
//	    	   if (!adapter.isEnabled()) {
//	    		    Intent enableBtIntent = new Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE);
//	    		    startActivity(enableBtIntent);
//	    		}
	       }
	 public static String getCar()
	 {
		return car.getText();
	 }
	 private final BroadcastReceiver mReceiver = new BroadcastReceiver(){
		@SuppressWarnings("unused")
		@Override
		public void onReceive(Context context, Intent intent) {
			 String action = intent.getAction();
			 BluetoothDevice device = intent.getParcelableExtra(BluetoothDevice.EXTRA_DEVICE);
			 String peas = device.getAddress();
			 String pods = car.getText();
			 NotificationCompat.Builder mBuilder2 =
			        new NotificationCompat.Builder(getBaseContext())
			        .setSmallIcon(R.drawable.ic_launcher)
			        .setContentTitle("I'm checking!")
			        .setContentText("Hello World!");
		 NotificationManager mNotificationManager =
				    (NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
		 mNotificationManager.notify(2, mBuilder2.build());
			 if(device.getAddress()== car.getText()){
				 NotificationCompat.Builder mBuilder =
					        new NotificationCompat.Builder(getBaseContext())
					        .setSmallIcon(R.drawable.ic_launcher)
					        .setContentTitle("found it!!!!!")
					        .setContentText("Hello World!");
				 NotificationManager mNotificationManager =
						    (NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
				 mNotificationManager.notify(1, mBuilder.build());
				 
			 
			 }
			 
		}
	 
	 };
//	    }
}

	   
