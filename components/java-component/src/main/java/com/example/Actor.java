package com.example;

import org.teavm.interop.Import;

public class Actor {
  // We treat Actor as a reference type.
  // In reality, this object's identity maps to the resource handle.
  // TeaVM Wasm GC should handle object passing if configured correctly,
  // but for WIT resources, it depends on the adapter.
  // Here we define the methods corresponding to the text.

  // Constructor implies creating a new resource, but here we are passed one.
  // So we don't need a constructor binding unless we call it.

  @Import(module = "spaghettikart:module/iactor",
          name = "[method]actor.outside-init")
  public native void
  outsideInit(float x, float y);

  @Import(module = "spaghettikart:module/iactor", name = "[method]actor.update")
  public native void update();

  @Import(module = "spaghettikart:module/iactor", name = "[method]actor.render")
  public native void render();

  @Import(module = "spaghettikart:module/iactor",
          name = "[method]actor.get-mod-id")
  public native String
  getModId();

  @Import(module = "spaghettikart:module/iactor", name = "[method]actor.get-id")
  public native String getId();

  @Import(module = "spaghettikart:module/iactor", name = "[method]actor.get-x")
  public native float getX();

  @Import(module = "spaghettikart:module/iactor", name = "[method]actor.set-x")
  public native void setX(float x);

  @Import(module = "spaghettikart:module/iactor", name = "[method]actor.get-y")
  public native float getY();

  @Import(module = "spaghettikart:module/iactor", name = "[method]actor.set-y")
  public native void setY(float y);
}
