use rust_web_ui::*;

pub struct Example12;

pub struct Example12Props {}

impl Default for Example12Props {
	fn default() -> Example12Props {
		Example12Props { }
	}
}

impl Widget<'_> for Example12 {
	type Props = Example12Props;

	fn render(mut ctx: Ctx<'_>, props: Example12Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Flex, Style::JustifyCenter, Style::Color(Color::Fg(11)), Style::Padding(Size::Exact(32))]);
			props.set_attribute("style", "min-height: 600px");
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Width(Size::Full), Style::Noop("max-w-xs")]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.set_attribute("x-data", r#"Components.listbox({ modelName: 'selected', open: true, selectedIndex: 3, activeIndex: 3, items: [{"id":1,"name":"Wade Cooper","avatar":"https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":2,"name":"Arlene Mccoy","avatar":"https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":3,"name":"Devon Webb","avatar":"https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80"},{"id":4,"name":"Tom Cook","avatar":"https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":5,"name":"Tanya Fox","avatar":"https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":6,"name":"Hellen Schmidt","avatar":"https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":7,"name":"Caroline Schultz","avatar":"https://images.unsplash.com/photo-1568409938619-12e139227838?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":8,"name":"Mason Heaney","avatar":"https://images.unsplash.com/photo-1531427186611-ecfd6d936c79?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":9,"name":"Claudie Smitham","avatar":"https://images.unsplash.com/photo-1584486520270-19eca1efcce5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":10,"name":"Emil Schaefer","avatar":"https://images.unsplash.com/photo-1561505457-3bcad021f8ee?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"}] })"#);
					props.set_attribute("x-init", "init()");
					
					props.child("1", Dynamic).run("label", |mut props| {
						props.set_attribute("id", "listbox-label");
						props.styles(&[Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
						props.set_attribute("@click", "$refs.button.focus()");
						
						props.child("0", Label).run(|props| props.text("Assigned to"));
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("relative"), Style::MarginTop(Size::Exact(8))]);
						
						props.child("1", Dynamic).run("button", |mut props| {
							props.set_attribute("type", "button");
							props.styles(&[Style::Noop("relative"), Style::Width(Size::Full), Style::Noop("cursor-default"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingY(Size::Exact(6)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(40)), Style::Noop("text-left"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-indigo-500")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
							props.set_attribute("x-ref", "button");
							props.set_attribute("@keydown.arrow-up.stop.prevent", "onButtonClick()");
							props.set_attribute("@keydown.arrow-down.stop.prevent", "onButtonClick()");
							props.set_attribute("@click", "onButtonClick()");
							props.set_attribute("aria-haspopup", "listbox");
							props.set_attribute(":aria-expanded", "open");
							props.set_attribute("aria-expanded", "true");
							props.set_attribute("aria-labelledby", "listbox-label");
							
							props.child("1", Dynamic).run("span", |mut props| {
								props.styles(&[Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("img", |mut props| {
									props.set_attribute(":src", "selected.avatar");
									props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
									props.set_attribute("alt", "");
									props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-text", "selected.name");
									props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
									
									props.child("0", Label).run(|props| props.text("Tom Cook"));
								});
							});
							props.child("3", Dynamic).run("span", |mut props| {
								props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::MarginLeft(Size::Exact(12)), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(8))]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::TextColor(Color::Fg(44))]);
								});
							});
						});
						props.child("3", Dynamic).run("ul", |mut props| {
							props.set_attribute("x-show", "open");
							props.set_attribute("x-transition:leave", "transition ease-in duration-100");
							props.set_attribute("x-transition:leave-start", "opacity-100");
							props.set_attribute("x-transition:leave-end", "opacity-0");
							props.styles(&[Style::Noop("absolute"), Style::Noop("z-10"), Style::MarginTop(Size::Exact(4)), Style::Noop("max-h-56"), Style::Width(Size::Full), Style::Noop("overflow-auto"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingY(Size::Exact(4)), Style::Noop("text-base"), Style::Noop("shadow-lg"), Style::Noop("ring-1"), Style::Noop("ring-black"), Style::Noop("ring-opacity-5"), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
							props.set_attribute("x-max", "1");
							props.set_attribute("@click.away", "open = false");
							props.set_attribute("x-description", "Select popover, show/hide based on select state.");
							props.set_attribute("@keydown.enter.stop.prevent", "onOptionSelect()");
							props.set_attribute("@keydown.space.stop.prevent", "onOptionSelect()");
							props.set_attribute("@keydown.escape", "onEscape()");
							props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
							props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
							props.set_attribute("x-ref", "listbox");
							props.set_attribute("tabindex", "-1");
							props.set_attribute("role", "listbox");
							props.set_attribute("aria-labelledby", "listbox-label");
							props.set_attribute(":aria-activedescendant", "activeDescendant");
							props.set_attribute("aria-activedescendant", "listbox-option-3");
							
							props.child("1", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-0");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(0)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 0)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 0, 'text-gray-900': !(activeIndex === 0) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 0, 'font-normal': !(selectedIndex === 0) }");
										
										props.child("0", Label).run(|props| props.text("Wade Cooper"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 0, 'text-indigo-600': !(activeIndex === 0) }");
									props.set_attribute("x-show", "selectedIndex === 0");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("3", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-1");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(1)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 1)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 1, 'text-gray-900': !(activeIndex === 1) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 1, 'font-normal': !(selectedIndex === 1) }");
										
										props.child("0", Label).run(|props| props.text("Arlene Mccoy"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 1, 'text-indigo-600': !(activeIndex === 1) }");
									props.set_attribute("x-show", "selectedIndex === 1");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("5", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-2");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(2)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 2)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 2, 'text-gray-900': !(activeIndex === 2) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 2, 'font-normal': !(selectedIndex === 2) }");
										
										props.child("0", Label).run(|props| props.text("Devon Webb"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 2, 'text-indigo-600': !(activeIndex === 2) }");
									props.set_attribute("x-show", "selectedIndex === 2");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("7", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-3");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(3)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 3)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 3, 'text-gray-900': !(activeIndex === 3) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 3, 'font-normal': !(selectedIndex === 3) }");
										
										props.child("0", Label).run(|props| props.text("Tom Cook"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 3, 'text-indigo-600': !(activeIndex === 3) }");
									props.set_attribute("x-show", "selectedIndex === 3");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("9", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-4");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(4)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 4)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 4, 'text-gray-900': !(activeIndex === 4) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 4, 'font-normal': !(selectedIndex === 4) }");
										
										props.child("0", Label).run(|props| props.text("Tanya Fox"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 4, 'text-indigo-600': !(activeIndex === 4) }");
									props.set_attribute("x-show", "selectedIndex === 4");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("11", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-5");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(5)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 5)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 5, 'text-gray-900': !(activeIndex === 5) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 5, 'font-normal': !(selectedIndex === 5) }");
										
										props.child("0", Label).run(|props| props.text("Hellen Schmidt"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 5, 'text-indigo-600': !(activeIndex === 5) }");
									props.set_attribute("x-show", "selectedIndex === 5");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("13", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-6");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(6)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 6)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 6, 'text-gray-900': !(activeIndex === 6) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1568409938619-12e139227838?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 6, 'font-normal': !(selectedIndex === 6) }");
										
										props.child("0", Label).run(|props| props.text("Caroline Schultz"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 6, 'text-indigo-600': !(activeIndex === 6) }");
									props.set_attribute("x-show", "selectedIndex === 6");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("15", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-7");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(7)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 7)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 7, 'text-gray-900': !(activeIndex === 7) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1531427186611-ecfd6d936c79?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 7, 'font-normal': !(selectedIndex === 7) }");
										
										props.child("0", Label).run(|props| props.text("Mason Heaney"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 7, 'text-indigo-600': !(activeIndex === 7) }");
									props.set_attribute("x-show", "selectedIndex === 7");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("17", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-8");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(8)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 8)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 8, 'text-gray-900': !(activeIndex === 8) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1584486520270-19eca1efcce5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 8, 'font-normal': !(selectedIndex === 8) }");
										
										props.child("0", Label).run(|props| props.text("Claudie Smitham"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 8, 'text-indigo-600': !(activeIndex === 8) }");
									props.set_attribute("x-show", "selectedIndex === 8");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
									});
								});
							});
							props.child("19", Dynamic).run("li", |mut props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), Style::Noop("relative"), Style::Noop("cursor-default"), Style::Noop("select-none"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(36))]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-9");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(9)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 9)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 9, 'text-gray-900': !(activeIndex === 9) }");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1561505457-3bcad021f8ee?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[Style::Noop("font-normal"), Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 9, 'font-normal': !(selectedIndex === 9) }");
										
										props.child("0", Label).run(|props| props.text("Emil Schaefer"));
									});
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[Style::Noop("text-indigo-600"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(16))]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 9, 'text-indigo-600': !(activeIndex === 9) }");
									props.set_attribute("x-show", "selectedIndex === 9");
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
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
