use rust_web_ui::*;

pub struct Example21;

pub struct Example21Props {}

impl Default for Example21Props {
	fn default() -> Example21Props {
		Example21Props { }
	}
}

impl Widget<'_> for Example21 {
	type Props = Example21Props;

	fn render(mut ctx: Ctx<'_>, props: Example21Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.set_attribute("x-data", "{ open: false }");
				props.set_attribute("@keydown.window.escape", "open = false");
				
				//  Mobile filter dialog 
				props.child("3", Dynamic).run("div", |mut props| {
					props.set_attribute("x-show", "open");
					props.styles(&[Style::Noop("relative"), Style::Noop("z-40"), Style::OnScreen(Screen::Large, &[Style::Noop("hidden")])]);
					props.set_attribute("x-description", "Off-canvas filters for mobile, show/hide based on off-canvas filters state.");
					props.set_attribute("x-ref", "dialog");
					props.set_attribute("aria-modal", "true");
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.set_attribute("x-show", "open");
						props.set_attribute("x-transition:enter", "transition-opacity ease-linear duration-300");
						props.set_attribute("x-transition:enter-start", "opacity-0");
						props.set_attribute("x-transition:enter-end", "opacity-100");
						props.set_attribute("x-transition:leave", "transition-opacity ease-linear duration-300");
						props.set_attribute("x-transition:leave-start", "opacity-100");
						props.set_attribute("x-transition:leave-end", "opacity-0");
						props.set_attribute("x-description", "Off-canvas menu backdrop, show/hide based on off-canvas menu state.");
						props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("bg-black"), Style::Noop("bg-opacity-25")]);
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("z-40"), Style::Flex]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.set_attribute("x-show", "open");
							props.set_attribute("x-transition:enter", "transition ease-in-out duration-300 transform");
							props.set_attribute("x-transition:enter-start", "translate-x-full");
							props.set_attribute("x-transition:enter-end", "translate-x-0");
							props.set_attribute("x-transition:leave", "transition ease-in-out duration-300 transform");
							props.set_attribute("x-transition:leave-start", "translate-x-0");
							props.set_attribute("x-transition:leave-end", "translate-x-full");
							props.set_attribute("x-description", "Off-canvas menu, show/hide based on off-canvas menu state.");
							props.styles(&[Style::Noop("relative"), Style::Noop("ml-auto"), Style::Flex, Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("max-w-xs"), Style::Noop("flex-col"), Style::Noop("overflow-y-auto"), Style::Noop("bg-white"), Style::PaddingY(Size::Exact(16)), Style::PaddingBottom(Size::Exact(48)), Style::Noop("shadow-xl")]);
							props.set_attribute("@click.away", "open = false");
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::PaddingX(Size::Exact(16))]);
								
								props.child("1", Dynamic).run("h2", |mut props| {
									props.styles(&[Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Filters"));
								});
								props.child("3", Dynamic).run("button", |mut props| {
									props.set_attribute("type", "button");
									props.styles(&[Style::Noop("-mr-2"), Style::Flex, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(44))]);
									props.set_attribute("@click", "open = false");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.text("Close menu"));
									});
									props.child("3", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
									});
								});
							});
							//  Filters 
							props.child("5", Dynamic).run("form", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("border-t"), Style::Noop("border-gray-200")]);
								
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("Categories"));
								});
								props.child("3", Dynamic).run("ul", |mut props| {
									props.set_attribute("role", "list");
									props.styles(&[Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12)), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12))]);
											
											props.child("0", Label).run(|props| props.text("Totes"));
										});
									});
									props.child("3", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12))]);
											
											props.child("0", Label).run(|props| props.text("Backpacks"));
										});
									});
									props.child("5", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12))]);
											
											props.child("0", Label).run(|props| props.text("Travel Bags"));
										});
									});
									props.child("7", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12))]);
											
											props.child("0", Label).run(|props| props.text("Hip Bags"));
										});
									});
									props.child("9", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12))]);
											
											props.child("0", Label).run(|props| props.text("Laptop Sleeves"));
										});
									});
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24))]);
									
									props.child("1", Dynamic).run("h3", |mut props| {
										props.styles(&[Style::Noop("-mx-2"), Style::Noop("-my-3"), Style::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |mut props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyBetween, Style::Noop("bg-white"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-mobile-0");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Color"));
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(24)), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
												props.child("3", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingTop(Size::Exact(24))]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-mobile-0");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-color-0");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "white");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-color-0");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("White"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-color-1");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "beige");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-color-1");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Beige"));
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-color-2");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "blue");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-color-2");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Blue"));
												});
											});
											props.child("7", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-color-3");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "brown");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-color-3");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Brown"));
												});
											});
											props.child("9", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-color-4");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "green");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-color-4");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Green"));
												});
											});
											props.child("11", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-color-5");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "purple");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-color-5");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Purple"));
												});
											});
										});
									});
								});
								props.child("7", Dynamic).run("div", |mut props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24))]);
									
									props.child("1", Dynamic).run("h3", |mut props| {
										props.styles(&[Style::Noop("-mx-2"), Style::Noop("-my-3"), Style::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |mut props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyBetween, Style::Noop("bg-white"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-mobile-1");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Category"));
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(24)), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
												props.child("3", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingTop(Size::Exact(24))]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-mobile-1");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-category-0");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "new-arrivals");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-category-0");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("New Arrivals"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-category-1");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "sale");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-category-1");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Sale"));
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-category-2");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "travel");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-category-2");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Travel"));
												});
											});
											props.child("7", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-category-3");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "organization");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-category-3");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Organization"));
												});
											});
											props.child("9", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-category-4");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "accessories");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-category-4");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Accessories"));
												});
											});
										});
									});
								});
								props.child("9", Dynamic).run("div", |mut props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24))]);
									
									props.child("1", Dynamic).run("h3", |mut props| {
										props.styles(&[Style::Noop("-mx-2"), Style::Noop("-my-3"), Style::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |mut props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyBetween, Style::Noop("bg-white"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(12)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-mobile-2");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Size"));
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(24)), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
												props.child("3", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingTop(Size::Exact(24))]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-mobile-2");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-size-0");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "2l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-size-0");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("2L"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-size-1");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "6l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-size-1");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("6L"));
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-size-2");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "12l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-size-2");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("12L"));
												});
											});
											props.child("7", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-size-3");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "18l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-size-3");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("18L"));
												});
											});
											props.child("9", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-size-4");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "20l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-size-4");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("20L"));
												});
											});
											props.child("11", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-mobile-size-5");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "40l");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-mobile-size-5");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("40L"));
												});
											});
										});
									});
								});
							});
						});
					});
				});
				props.child("5", Dynamic).run("main", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Flex, Style::Noop("items-baseline"), Style::JustifyBetween, Style::Noop("border-b"), Style::Noop("border-gray-200"), Style::PaddingBottom(Size::Exact(24)), Style::PaddingTop(Size::Exact(96))]);
						
						props.child("1", Dynamic).run("h1", |mut props| {
							props.styles(&[Style::Noop("text-4xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("New Arrivals"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.set_attribute("x-data", "Components.menu({ open: false })");
								props.set_attribute("x-init", "init()");
								props.set_attribute("@keydown.escape.stop", "open = false; focusButton()");
								props.set_attribute("@click.away", "onClickAway($event)");
								props.styles(&[Style::Noop("relative"), Style::InlineBlock, Style::Noop("text-left")]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.child("1", Dynamic).run("button", |mut props| {
										props.set_attribute("type", "button");
										props.styles(&[Style::Noop("group"), Style::InlineFlex, Style::JustifyCenter, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::TextColor(Color::Fg(100))])]);
										props.set_attribute("id", "menu-button");
										props.set_attribute("x-ref", "button");
										props.set_attribute("@click", "onButtonClick()");
										props.set_attribute("@keyup.space.prevent", "onButtonEnter()");
										props.set_attribute("@keydown.enter.prevent", "onButtonEnter()");
										props.set_attribute("aria-expanded", "false");
										props.set_attribute("aria-haspopup", "true");
										props.set_attribute("x-bind:aria-expanded", "open.toString()");
										props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
										props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
										
										props.child("0", Label).run(|props| props.text("Sort"));
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Noop("-mr-1"), Style::MarginLeft(Size::Exact(4)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44)), Style::NoopGroup("group-hover", &[Style::TextColor(Color::Fg(56))])]);
										});
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.set_attribute("x-show", "open");
									props.set_attribute("x-transition:enter", "transition ease-out duration-100");
									props.set_attribute("x-transition:enter-start", "transform opacity-0 scale-95");
									props.set_attribute("x-transition:enter-end", "transform opacity-100 scale-100");
									props.set_attribute("x-transition:leave", "transition ease-in duration-75");
									props.set_attribute("x-transition:leave-start", "transform opacity-100 scale-100");
									props.set_attribute("x-transition:leave-end", "transform opacity-0 scale-95");
									props.styles(&[Style::Noop("absolute"), Style::Noop("right-0"), Style::Noop("z-10"), Style::MarginTop(Size::Exact(8)), Style::Width(Size::Exact(160)), Style::Noop("origin-top-right"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::Noop("shadow-2xl"), Style::Noop("ring-1"), Style::Noop("ring-black"), Style::Noop("ring-opacity-5"), Style::OnFocus(&[Style::Noop("outline-none")])]);
									props.set_attribute("x-ref", "menu-items");
									props.set_attribute("x-description", "Dropdown menu, show/hide based on menu state.");
									props.set_attribute("x-bind:aria-activedescendant", "activeDescendant");
									props.set_attribute("role", "menu");
									props.set_attribute("aria-orientation", "vertical");
									props.set_attribute("aria-labelledby", "menu-button");
									props.set_attribute("tabindex", "-1");
									props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
									props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
									props.set_attribute("@keydown.tab", "open = false");
									props.set_attribute("@keydown.enter.prevent", "open = false; focusButton()");
									props.set_attribute("@keyup.space.prevent", "open = false; focusButton()");
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingY(Size::Exact(4))]);
										props.set_attribute("role", "none");
										
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100)), Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm")]);
											props.set_attribute("x-state:on", "Active");
											props.set_attribute("x-state:off", "Not Active");
											props.set_attribute("x-state:on:option.current", "Selected");
											props.set_attribute("x-state:off:option.current", "Not Selected");
											props.set_attribute("x-state-description", r#"Selected: "font-medium text-gray-900", Not Selected: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 0 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-0");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 0)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.text("Most Popular"));
										});
										props.child("3", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::TextColor(Color::Fg(56)), Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm")]);
											props.set_attribute("x-state-description", r#"undefined: "font-medium text-gray-900", undefined: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 1 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-1");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 1)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.text("Best Rating"));
										});
										props.child("5", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::TextColor(Color::Fg(56)), Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm")]);
											props.set_attribute("x-state-description", r#"undefined: "font-medium text-gray-900", undefined: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 2 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-2");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 2)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.text("Newest"));
										});
										props.child("7", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::TextColor(Color::Fg(56)), Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm")]);
											props.set_attribute("x-state-description", r#"undefined: "font-medium text-gray-900", undefined: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 3 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-3");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 3)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.text("Price: Low to High"));
										});
										props.child("9", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::TextColor(Color::Fg(56)), Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm")]);
											props.set_attribute("x-state-description", r#"undefined: "font-medium text-gray-900", undefined: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 4 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-4");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 4)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.text("Price: High to Low"));
										});
									});
								});
							});
							props.child("3", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Noop("-m-2"), Style::MarginLeft(Size::Exact(20)), Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))]), Style::OnScreen(Screen::Small, &[Style::MarginLeft(Size::Exact(28))])]);
								
								props.child("1", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("View grid"));
								});
								props.child("3", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
								});
							});
							props.child("5", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Noop("-m-2"), Style::MarginLeft(Size::Exact(16)), Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))]), Style::OnScreen(Screen::Small, &[Style::MarginLeft(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::Noop("hidden")])]);
								props.set_attribute("@click", "open = true");
								
								props.child("1", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("Filters"));
								});
								props.child("3", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
								});
							});
						});
					});
					props.child("3", Dynamic).run("section", |mut props| {
						props.set_attribute("aria-labelledby", "products-heading");
						props.styles(&[Style::PaddingBottom(Size::Exact(96)), Style::PaddingTop(Size::Exact(24))]);
						
						props.child("1", Dynamic).run("h2", |mut props| {
							props.set_attribute("id", "products-heading");
							props.styles(&[Style::Noop("sr-only")]);
							
							props.child("0", Label).run(|props| props.text("Products"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-10"), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-4")])]);
							
							//  Filters 
							props.child("3", Dynamic).run("form", |mut props| {
								props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Large, &[Style::Block])]);
								
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("Categories"));
								});
								props.child("3", Dynamic).run("ul", |mut props| {
									props.set_attribute("role", "list");
									props.styles(&[Style::SpaceY(Size::Exact(16)), Style::Noop("border-b"), Style::Noop("border-gray-200"), Style::PaddingBottom(Size::Exact(24)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.text("Totes"));
										});
									});
									props.child("3", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.text("Backpacks"));
										});
									});
									props.child("5", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.text("Travel Bags"));
										});
									});
									props.child("7", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.text("Hip Bags"));
										});
									});
									props.child("9", Dynamic).run("li", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.text("Laptop Sleeves"));
										});
									});
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[Style::Noop("border-b"), Style::Noop("border-gray-200"), Style::PaddingY(Size::Exact(24))]);
									
									props.child("1", Dynamic).run("h3", |mut props| {
										props.styles(&[Style::Noop("-my-3"), Style::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |mut props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyBetween, Style::Noop("bg-white"), Style::PaddingY(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-0");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Color"));
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(24)), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
												props.child("3", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingTop(Size::Exact(24))]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-0");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::SpaceY(Size::Exact(16))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-color-0");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "white");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-color-0");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("White"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-color-1");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "beige");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-color-1");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Beige"));
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-color-2");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "blue");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-color-2");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Blue"));
												});
											});
											props.child("7", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-color-3");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "brown");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-color-3");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Brown"));
												});
											});
											props.child("9", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-color-4");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "green");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-color-4");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Green"));
												});
											});
											props.child("11", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-color-5");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "purple");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-color-5");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Purple"));
												});
											});
										});
									});
								});
								props.child("7", Dynamic).run("div", |mut props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[Style::Noop("border-b"), Style::Noop("border-gray-200"), Style::PaddingY(Size::Exact(24))]);
									
									props.child("1", Dynamic).run("h3", |mut props| {
										props.styles(&[Style::Noop("-my-3"), Style::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |mut props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyBetween, Style::Noop("bg-white"), Style::PaddingY(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-1");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Category"));
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(24)), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
												props.child("3", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingTop(Size::Exact(24))]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-1");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::SpaceY(Size::Exact(16))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-category-0");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "new-arrivals");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-category-0");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("New Arrivals"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-category-1");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "sale");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-category-1");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Sale"));
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-category-2");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "travel");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-category-2");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Travel"));
												});
											});
											props.child("7", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-category-3");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "organization");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-category-3");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Organization"));
												});
											});
											props.child("9", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-category-4");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "accessories");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-category-4");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("Accessories"));
												});
											});
										});
									});
								});
								props.child("9", Dynamic).run("div", |mut props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[Style::Noop("border-b"), Style::Noop("border-gray-200"), Style::PaddingY(Size::Exact(24))]);
									
									props.child("1", Dynamic).run("h3", |mut props| {
										props.styles(&[Style::Noop("-my-3"), Style::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |mut props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyBetween, Style::Noop("bg-white"), Style::PaddingY(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-2");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Size"));
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(24)), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
												props.child("3", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingTop(Size::Exact(24))]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-2");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::SpaceY(Size::Exact(16))]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-size-0");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "2l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-size-0");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("2L"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-size-1");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "6l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-size-1");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("6L"));
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-size-2");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "12l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-size-2");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("12L"));
												});
											});
											props.child("7", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-size-3");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "18l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-size-3");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("18L"));
												});
											});
											props.child("9", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-size-4");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "20l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-size-4");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("20L"));
												});
											});
											props.child("11", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |mut props| {
													props.set_attribute("id", "filter-size-5");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "40l");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Rounded, Style::Noop("border-gray-300"), Style::Noop("text-indigo-600"), Style::OnFocus(&[Style::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |mut props| {
													props.set_attribute("for", "filter-size-5");
													props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.text("40L"));
												});
											});
										});
									});
								});
							});
							//  Product grid 
							props.child("7", Dynamic).run("div", |mut props| {
								props.styles(&[Style::OnScreen(Screen::Large, &[Style::Noop("col-span-3")])]);
								
								props.child("1", Dynamic).run("x-placeholder", |mut props| {
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("relative"), Style::Width(Size::Exact(384)), Style::Noop("overflow-hidden"), Style::Noop("rounded-xl"), Style::Noop("border"), Style::Noop("border-dashed"), Style::Noop("border-gray-400"), Style::Noop("opacity-75"), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("stroke-gray-900/10")]);
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
