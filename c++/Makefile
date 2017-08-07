CXX = g++-5
CXXFLAGS = -std=c++14 -MMD -Wall
SOURCES = $(shell find src -name *.cc)
OBJECTS = ${SOURCES:.cc=.o}
DEPENDS = ${OBJECTS:.o=.d}
EXEC = roguelike
${EXEC}: ${OBJECTS}
	@echo Linking executable
	@${CXX} ${CXXFLAGS} ${OBJECTS} -o ${EXEC} -lncursesw
	@echo Done!
.PHONY: clean clean-dep list test
clean:
	-@rm ${OBJECTS} ${DEPENDS} ${EXEC} 2> /dev/null || true
clean-dep:
	-@rm ${OBJECTS} ${DEPENDS} 2> /dev/null || true
list:
	@echo ${OBJECTS}
