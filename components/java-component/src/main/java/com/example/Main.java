package com.example;

import org.teavm.interop.Export;
import org.teavm.interop.Import;

public class Main {

  public static void main(String[] args) {}

  @Export(name = "init")
  public static void init() {
    System.out.println("Java: init()");
  }

  @Export(name = "actor-init")
  public static void actorInit(Actor a, float x, float y) {
    System.out.println("Java: actor-init");
    a.outsideInit(x, y);
  }

  @Export(name = "actor-update")
  public static void actorUpdate(Actor a) {
    a.update();
  }

  @Export(name = "actor-render")
  public static void actorRender(Actor a) {
    a.render();
  }
}
