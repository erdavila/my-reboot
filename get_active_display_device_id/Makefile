EXE = get_active_display_device_id
CXXFLAGS = -mwindows
DIST = ../dist/$(EXE)


.PHONY: all
all: $(DIST)

$(DIST): $(EXE)
	cp $< $@

%.exe: %.cc
	$(CXX) $(CXXFLAGS) $< -o $@

.PHONY: clean
clean:
	rm -f *.exe

# The following targets are here only to help Bash auto-completion :-)
get_active_display_device_id:
EnumDisplayDevices:
