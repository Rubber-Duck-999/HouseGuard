package com;

public class StubConsumerTopic extends ConsumerTopic
{
    private boolean accessAllowed;
    private boolean accessStateSet;
    
    @Override
    public boolean getAccessState()
    {
    	return accessAllowed;
    }
    
    public void setAccessAllowed(boolean accessAllowed) 
    {
		this.accessAllowed = accessAllowed;
	}

	public void setAccessStateSet(boolean accessStateSet) 
	{
		this.accessStateSet = accessStateSet;
	}

	@Override
    public boolean getAccessStateSet()
    {
    	return accessStateSet;
    }
    
    @Override
    public void setAccessStateSetOff()
    {
    	accessStateSet = false;
    }

    @Override
    public void askForAccess(Integer key, Integer val)
    {
        System.out.println("askForAccess Call : " + key + ", " + val);
    }

    @Override
	public void consumeRequired()
    {
		System.out.println("consumeRequired Call");	
    }
    
    public StubConsumerTopic()
    {
    	accessAllowed = false;
    	accessStateSet = false;
    	System.out.println("Constructor Call");	
    }

    @Override
	public Integer getId() 
	{
		return 0;		
	}
}