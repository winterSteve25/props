module.export = grammar({
	name: "props",
	rules: {
		externals: $ => [
			$.indent,
			$.dedent,
			$.newline,
		]
	}
})
